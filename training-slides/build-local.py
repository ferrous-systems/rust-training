#!/usr/bin/env python3
import argparse
import threading
import subprocess
import sys
import webbrowser
import socket
import time
from pathlib import Path


def build_book() -> None:
    print("Building book with mdBook…")
    subprocess.run(["mdbook", "build"], check=True)


def build_slides() -> None:
    print("Building slides with mdslides…")
    subprocess.run(
        [
            "mdslides",
            "--template",
            "./template.html",
            "--output-dir",
            "./slides",
            "--mdbook-path",
            ".",
            "--index-template",
            "./index-template.html",
        ],
        check=True,
    )


def open_book() -> None:
    index = Path("./book/index.html").resolve()
    if not index.exists():
        print(
            f"Warning: {index} does not exist. Did you build the book?", file=sys.stderr
        )
    url = index.as_uri()
    print(f"Opening book in browser: {url}")
    webbrowser.open(url)


def open_slides(port: int = 8000) -> None:
    url = f"http://localhost:{port}/"
    print(f"Opening slides in browser: {url}")
    webbrowser.open(url)


def wait_for_port(port: int, host: str = "127.0.0.1", timeout: float = 5.0) -> bool:
    deadline = time.time() + timeout
    while time.time() < deadline:
        with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
            s.settimeout(0.3)
            if s.connect_ex((host, port)) == 0:
                return True
        time.sleep(0.1)
    return False


def serve_slides_foreground(open_after_start: bool = False, port: int = 8000) -> None:
    slides_dir = Path("./slides")
    if not slides_dir.exists():
        print(
            "Warning: ./slides does not exist. You may want to build slides first with -s.",
            file=sys.stderr,
        )

    print(f"Serving ./slides at http://localhost:{port}/ (Ctrl+C to stop)…")
    proc = subprocess.Popen(
        [sys.executable, "-m", "http.server", str(port), "-d", "./slides"]
    )

    try:
        if open_after_start:
            # Wait briefly until the server is ready, then open the browser
            if wait_for_port(port, timeout=5.0):
                open_slides(port)
            else:
                print(
                    "Warning: server did not become ready in time to open the browser.",
                    file=sys.stderr,
                )

        proc.wait()
    except KeyboardInterrupt:
        print("\nStopping slides server…")
        try:
            proc.terminate()
            proc.wait(timeout=5)
        except subprocess.TimeoutExpired:
            proc.kill()
            proc.wait()
        print("Slides server stopped.")


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Build mdBook and slides, and optionally open them in a browser."
    )
    parser.add_argument(
        "-b",
        "--book",
        action="store_true",
        help="Build the book (mdbook build).",
    )
    parser.add_argument(
        "-s",
        "--slides",
        action="store_true",
        help="Build the slides (mdslides …).",
    )
    parser.add_argument(
        "--serve",
        action="store_true",
        help="Serve slides in the foreground (http://localhost:8000).",
    )
    parser.add_argument(
        "-o",
        "--open",
        action="store_true",
        help="Open the built book and/or slides in a browser.",
    )
    return parser.parse_args()


def main() -> None:
    args = parse_args()

    # Default: if neither -b nor -s provided, do both (matches original script behavior).
    do_book = args.book or (not args.book and not args.slides)
    do_slides = args.slides or (not args.book and not args.slides)

    try:
        if do_book:
            build_book()
        if do_slides:
            build_slides()

        # Open requested targets
        if args.open:
            if do_book:
                open_book()
            if do_slides and not args.serve:
                print(
                    "Note: slides are opened at http://localhost:8000/. Make sure a server is running (use --serve)."
                )
                open_slides(port=8000)

        # Serve slides in the foreground (blocks; Ctrl+C to stop)
        if args.serve:
            serve_slides_foreground(
                open_after_start=(args.open and do_slides), port=8000
            )

    except KeyboardInterrupt:
        # Clean, unified Ctrl+C handling for non-server operations
        print("\nInterrupted by user.")
        sys.exit(130)
    except subprocess.CalledProcessError as e:
        print(
            f"Error: command failed with exit code {e.returncode}: {e.cmd}",
            file=sys.stderr,
        )
        sys.exit(e.returncode)


if __name__ == "__main__":
    main()
