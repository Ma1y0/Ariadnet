#!/usr/bin/env python

import socket
import time
import argparse
import threading
import errno
from concurrent.futures import ThreadPoolExecutor
from collections import Counter

class Stats:
    def __init__(self):
        self.counter = Counter()
        self.lock = threading.Lock()
        self.error_details = []

    def increment(self, status):
        with self.lock:
            self.counter[status] += 1

    def add_error_detail(self, error):
        with self.lock:
            self.error_details.append(error)

    def get_stats(self):
        return dict(self.counter)

    def get_error_details(self):
        return self.error_details


stats = Stats()


def send_message(thread_id):
    # Create a TCP socket
    try:
        client_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        client_socket.settimeout(5)  # 5 second timeout
    except Exception as e:
        stats.increment("socket_creation_error")
        stats.add_error_detail(f"Socket creation error: {str(e)}")
        return

    try:
        # Connect to the server
        client_socket.connect(("localhost", 8081))

        # Prepare the message
        message = "1 GET /\n\n\nHello World"

        # Send the message
        bytes_sent = client_socket.send(message.encode())

        if bytes_sent == len(message):
            stats.increment("success")
        else:
            stats.increment("partial_send")
            stats.add_error_detail(
                f"Partial send: {bytes_sent} of {len(message)} bytes"
            )

    except ConnectionRefusedError:
        stats.increment("connection_refused")
    except socket.timeout:
        stats.increment("timeout")
    except OSError as e:
        if e.errno == errno.EMFILE:  # Too many open files
            stats.increment("too_many_files")
        elif e.errno == errno.EADDRINUSE:  # Address already in use
            stats.increment("address_in_use")
        elif e.errno == errno.EADDRNOTAVAIL:  # Cannot assign requested address
            stats.increment("no_local_ports")
        else:
            stats.increment("other_os_error")
            stats.add_error_detail(f"OS Error {e.errno}: {str(e)}")
    except Exception as e:
        stats.increment("other_error")
        stats.add_error_detail(f"Unexpected error: {str(e)}")
    finally:
        try:
            client_socket.close()
        except:
            pass  # Ignore errors during close


def main():
    parser = argparse.ArgumentParser(description="Send TCP messages in parallel")
    parser.add_argument(
        "--threads",
        type=int,
        default=10,
        help="Number of parallel threads (default: 10)",
    )
    parser.add_argument(
        "--delay",
        type=float,
        default=0,
        help="Delay between thread launches in seconds (default: 0)",
    )
    args = parser.parse_args()

    print(f"Starting {args.threads} parallel connections...")

    start_time = time.time()

    with ThreadPoolExecutor(max_workers=args.threads) as executor:
        for i in range(args.threads):
            executor.submit(send_message, i)
            if args.delay > 0:
                time.sleep(args.delay)

    end_time = time.time()
    duration = end_time - start_time

    # Get and print statistics
    final_stats = stats.get_stats()
    print("\n=== Final Statistics ===")
    print(f"Total attempts: {args.threads}")
    print(f"Successful: {final_stats.get('success', 0)}")
    print(f"Failed - Connection Refused: {final_stats.get('connection_refused', 0)}")
    print(f"Failed - Timeout: {final_stats.get('timeout', 0)}")
    print(f"Failed - Too Many Open Files: {final_stats.get('too_many_files', 0)}")
    print(f"Failed - No Local Ports Available: {final_stats.get('no_local_ports', 0)}")
    print(f"Failed - Address Already in Use: {final_stats.get('address_in_use', 0)}")
    print(f"Failed - Other OS Errors: {final_stats.get('other_os_error', 0)}")
    print(f"Failed - Other Errors: {final_stats.get('other_error', 0)}")
    print(f"Failed - Socket Creation: {final_stats.get('socket_creation_error', 0)}")
    print(f"Partial Sends: {final_stats.get('partial_send', 0)}")
    print(f"Total dropped: {args.threads - final_stats.get('success', 0)}")
    print(f"Time taken: {duration:.2f} seconds")
    print(f"Requests per second: {args.threads / duration:.2f}")

    # Print first few detailed errors if any
    error_details = stats.get_error_details()
    if error_details:
        print("\n=== First 10 Error Details ===")
        for error in error_details[:10]:
            print(error)


if __name__ == "__main__":
    main()
