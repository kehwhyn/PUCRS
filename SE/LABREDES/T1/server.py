#!/usr/bin/env python3
# -*- coding: utf-8 -*-

import socket
from random import randrange

HOST = ''
PORT = 50008
MSG_SIZE = 1024


def play(sock, addr):
    QUESTIONS = [
        "Interligam redes que diferem bastante entre si",
        "Solução para o problema do IPv4",
        "Protocolo mais popular da internet",
        "Protocolo de emails",
        "O que TCP garante?",
        "Em que tipo de aplicações é usado UDP?",
        "Quem define o Default Gateway?",
        "O que DCHP fornece?",
        "Que protocolo converte nome de máquina para seu endereço IP?",
        "Forma de roteamento entre máquinas de diferentes redes",
    ]
    ANSWERS = [
        "roteadores",
        "network address translator",
        "http",
        "simple message trasfer protocol",
        "transferencia de dados",
        "entrega imediata",
        "administrador da rede",
        "enderecos ip temporarios",
        "domain name system",
        "indireto"
    ]

    selected_word_index = randrange(len(QUESTIONS))
    chosen_word = ANSWERS[selected_word_index].split(" ")
    retries = 5

    known_letters = [ ["_"] * len(word)
        for word in chosen_word
    ]

    got_it = False
    while retries > 0 and not got_it:

        msg = f"Número restante de chances: {retries}\n"
        msg += QUESTIONS[selected_word_index] + "\n"
        for palavra in known_letters:
            msg += "".join(palavra) + " "
        msg += "\n\n"
        msg += "Digite uma letra: "
        sock.sendto(msg.encode(), addr)

        data, addr = sock.recvfrom(MSG_SIZE)
        data = data.decode().lower()

        if data.isalpha():
            if len(data) == 1:

                letra_encontrada = False
                for ind, palavra in enumerate(chosen_word):
                    for indice, caractere in enumerate(palavra):
                        if data == caractere:
                            letra_encontrada = True
                            known_letters[ind][indice] = data

                if not letra_encontrada:
                    retries -= 1

                aux = ""
                for palavra in known_letters:
                    aux += "".join(palavra) + " "
                aux = aux.strip()
                got_it = aux == ANSWERS[selected_word_index]

            else:
                got_it = data == ANSWERS[selected_word_index]
                if not got_it:
                    retries -= 1

    if retries == 0:
        msg = "\nAcabaram-se as chances :(\n Fim do jogo! x.x\n"
    if got_it:
        msg = f"\nParabéns, você adivinhou a palavra! :) \n{ANSWERS[selected_word_index]}\n Fim do jogo! ^.^\n"
    sock.sendto(msg.encode(), addr)

    return addr


def main():
    with socket.socket(socket.AF_INET, socket.SOCK_DGRAM) as sock:

        sock.bind((HOST, PORT))

        while True:
            _, addr = sock.recvfrom(MSG_SIZE)
            addr = play(sock, addr)


if __name__ == "__main__":
    main()