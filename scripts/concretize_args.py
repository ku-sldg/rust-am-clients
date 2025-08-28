import re
import sys
import argparse
import pathlib
import string

def parse_key_value_pair(item):
    """Parses a string of the form: 'key1=val1' into a dictionary."""
    #print("keyval: ")
    #print(item)
    result = {}
    if '=' not in item:
        raise argparse.ArgumentTypeError(f"Invalid argument: {item}. Expected format key=value")
    key, value = item.split('=', 1)
    result[key] = value
    return result

def replace_strings_in_file(input_file, output_file, replacement_map, special_chars):
    """
    Reads a file, replaces strings enclosed in special characters, and writes to a new file.

    Args:
        input_file (str): Path to the input file.
        output_file (str): Path to the output file.
        replacement_map (dict): Dictionary of strings to replace (keys) and their replacements (values).
        special_chars (str): Special characters enclosing the strings to replace (e.g., '$$').
    """
    try:
        with open(input_file, 'r') as infile, open(output_file, 'w') as outfile:
            for line in infile:
                def replace_match(match):
                    key = match.group(1)
                    return replacement_map.get(key, match.group(0))
                
                pattern = re.compile(re.escape(special_chars[0]) + r'(.*?)' + re.escape(special_chars[1]))
                modified_line = pattern.sub(replace_match, line)
                outfile.write(modified_line)
        print(f"\nFile processed successfully. Output written to: {output_file}\n")

    except FileNotFoundError:
        print(f"Error: Input file not found: {input_file}")
    except Exception as e:
        print(f"An error occurred: {e}")

if __name__ == "__main__":

    special_characters = ['$$', '$$']

    parser = argparse.ArgumentParser(description="Parser for concretize_args.py CLI args.")
    parser.add_argument(
        '--params',
        type=parse_key_value_pair,
        nargs='+',
        help='A map of key-value pairs (e.g., --params key1=value1 key2=value2)')
    
    parser.add_argument(
    '--infile',
    help='Path to input file')

    parser.add_argument(
    '--outfile',
    help='Path to (new) output file')

    args = parser.parse_args()

    input_file_path = args.infile
    output_file_path = args.outfile

    print ("\ninput_file_path:")
    print ("\t", input_file_path)

    print ("\noutput_file_path:")
    print ("\t", output_file_path)

    replacements = {}
    for d in args.params:
            replacements.update(d)

    print("\nreplacement strings map:")
    print("\t", replacements)
    
    replace_strings_in_file(input_file_path, output_file_path, replacements, special_characters)