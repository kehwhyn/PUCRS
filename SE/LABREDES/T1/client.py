#!/usr/bin/env python3
# -*- coding: utf-8 -*-

# client program using udp
import socket

HOST = '127.0.0.1'
PORT = 50008
ADDR = (HOST, PORT)
MSG_SIZE = 1024

INTRO = "Olá, bem-vindx(s) ao Jogo da Forca!"
OPTIONS = """
1 - Regras
2 - play
3 - Multiplayer
4 - Parar de play
"""
RULES = """
O jogo é baseado em temas/perguntas e respostas.
Você recebe uma pergunta e tem que adivinhar qual a reposta.
O número de tentativas é 5, caso passe de disso você perde o jogo.
5 é o mesmo número de partes do corpo que desenhamos no papel quando jogamos a forca.
Só letras erradas diminuem o número de tentativas.
Caso você queria adivinhar a palavra, você pode fazer isso. Errar só diminui as tentativas.
"""


def play():
    with socket.socket(socket.AF_INET, socket.SOCK_DGRAM) as sock:
        sock.sendto("primeira conexão com o servidor".encode(), ADDR)
        while True:
            data, _ = sock.recvfrom(MSG_SIZE)
            data = data.decode()

            if "Fim do jogo!" in data:
                print(data)
                break

            resp = input(data)
            sock.sendto(resp.encode(), ADDR)


def main():
    while True:
        print(INTRO)
        print(OPTIONS)
        opcao = input("Digite sua opção: ")

        if opcao == "1":
            print(RULES)
        elif opcao == "2":
            print("Vamos jogar então!")
            play()
        elif opcao == "3":
            print("not implemented yet.")
        elif opcao == "4":
            print("Obrigado por jogar! <3")
            break
        else:
            print("Opção inválida, por favor digite novamente.")


if __name__ == "__main__":
    main()