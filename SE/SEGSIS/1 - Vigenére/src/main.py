from pathlib import Path
from collections import Counter
from concurrent.futures import ThreadPoolExecutor

ALPHABET = 'abcdefghijklmnopqrstuvwxyz'

CIPHER_TEXTS_DIR = Path("../textos-cifrados-20230312")
TEST_ENGLISH = "../test/cipher_text_english.txt"
TEST_PORTUGUESE = "../test/texto_cifrado_portugues.txt"

# portuguese sorted 5 most frequent letters
PT_FMFL = "aeors"

# english sorted 5 most frequent letters
EN_FMFL = "aeiot"

def index_of_coincidence(word):
    coincidence_index = 0
    n = len(word)
    occurrences = list(Counter(word).values())
    for occurrence in occurrences:
        coincidence_index += occurrence / n * (occurrence - 1) / (n - 1)
    return round(coincidence_index, 3)


def find_key_length(cipher_text):
    key_length = 1
    while key_length < (len(cipher_text) / 2):
        
        substrings = [cipher_text[i::key_length] for i in range(key_length)]
        indexes = [index_of_coincidence(substring) for substring in substrings]
    
        diff = round(abs(sum(indexes) / len(indexes) - 0.067), 3)
        if 0 <= diff <= 0.002:
            break

        diff = round(abs(sum(indexes) / len(indexes) - 0.081), 3)
        if 0 <= diff <= 0.005:
            break

        key_length += 1
    
    return key_length


def generate_possible_keys(cipher_text, key_length):
    substrings = [cipher_text[i::key_length] for i in range(key_length)]
    occurrences = [[key for key, _ in Counter(substring).most_common()] for substring in substrings]
    possible_keys = ["".join([occurrence[index] for occurrence in occurrences if index < len(occurrence)]) for index in range(len(ALPHABET))]
    return possible_keys[:5]


def decrypt(cipher_text, possible_keys, key_length):
    chunks = [cipher_text[index : index + key_length] for index in range(0, len(cipher_text), key_length)]
    
    plaintext = ""
    for key in possible_keys:
        for chunk in chunks:
            distances = [ord(char) - ord(dist) for char, dist in zip(chunk, key)]
            plaintext += "".join([ALPHABET[distance] for distance in distances])
    
        fmfl = "".join(sorted([key for key, _ in Counter(plaintext).most_common(5)]))

        if fmfl in [PT_FMFL, EN_FMFL]:
            break
         
        plaintext = ""
            
    return plaintext


def run_test_english():
    with open(TEST_ENGLISH, "r") as file:
        cipher_text = file.read()

    key_length = find_key_length(cipher_text)

    possible_keys = generate_possible_keys(cipher_text, key_length)

    plaintext = decrypt(cipher_text, possible_keys, key_length)

    with open(f"../out/{TEST_ENGLISH[8:]}", "w") as file:
        file.write(plaintext)


def roda_teste_portugues():
    with open(TEST_PORTUGUESE, "r") as file:
        cipher_text = file.read()

    key_length = find_key_length(cipher_text)

    possible_keys = generate_possible_keys(cipher_text, key_length)
    
    plaintext = decrypt(cipher_text, ["meunome"], key_length)
    
    with open(f"../out/{TEST_PORTUGUESE[8:]}", "w") as file:
        file.write(plaintext)


def run_all_tests(cipher_text_file):
    with open(cipher_text_file, "r") as file:
        cipher_text = file.read()

    key_length = find_key_length(cipher_text)

    possible_keys = generate_possible_keys(cipher_text, key_length)
    
    plaintext = decrypt(cipher_text, possible_keys, key_length)
    
    with open(f"../out/{cipher_text_file.name}", "w") as file:
        file.write(plaintext)


def main():
    run_test_english()
    roda_teste_portugues()

    with ThreadPoolExecutor() as executor:
        threads = []
        for cipher_text_file in CIPHER_TEXTS_DIR.iterdir():
            thread = executor.submit(run_all_tests, cipher_text_file)
            threads.append(thread)


if __name__ == "__main__":
    main()