#!/usr/bin/env python3

import argparse
import sys


def convert_to_tsv(input_file_path, output_file_path):
    """
    Converts a file from '<word> <frequency>' format to '<word>\t<frequency>' TSV format.
    Handles extra whitespace and skips malformed lines.
    """
    try:
        with open(input_file_path, "r", encoding="utf-8") as infile, open(
            output_file_path, "w", encoding="utf-8"
        ) as outfile:
            for line in infile:
                line = line.strip()
                if not line:
                    continue
                # Split on the first whitespace
                parts = line.split(None, 1)
                if len(parts) == 2:
                    word, frequency = parts
                    word = word.strip()
                    frequency = frequency.strip()
                    if word and frequency.isdigit():
                        outfile.write(f"{word}\t{frequency}\n")
                    else:
                        print(f"Skipping malformed line: {line}", file=sys.stderr)
                else:
                    print(f"Skipping malformed line: {line}", file=sys.stderr)
        print(f"Successfully converted '{input_file_path}' to '{output_file_path}'")
    except FileNotFoundError:
        print(f"Error: Input file '{input_file_path}' not found.", file=sys.stderr)
        sys.exit(1)
    except Exception as e:
        print(f"An error occurred: {e}", file=sys.stderr)
        sys.exit(1)


if __name__ == "__main__":
    parser = argparse.ArgumentParser(
        description="Convert a word frequency file to TSV format."
    )
    parser.add_argument(
        "input_file", help="Path to the input file (format: word frequency)"
    )
    parser.add_argument(
        "output_file", help="Path to the output TSV file (format: word\tfrequency)"
    )
    args = parser.parse_args()

    convert_to_tsv(args.input_file, args.output_file)
