#include <stdio.h>
#include <stdlib.h>
#include <sys/types.h>
#include <sys/wait.h>
#include <unistd.h>

void mostrar_menu();

int main() {
  char opcion;
  int status;

  while (1) {
    mostrar_menu();
    scanf(" %c", &opcion);

    pid_t pid;

    if (opcion == 'a' || opcion == 'b' || opcion == 'c') {
      printf("Ejecutando...\n");
      if (opcion == 'a') {
        pid = fork();
        if (pid == 0) {
          execlp("ls", "ls", "-l", NULL);
        } else if (pid > 0) {
          wait(&status);
        }
      } else if (opcion == 'b') {
        pid = fork();
        if (pid == 0) {
          execl("./bin/leeArchivo", "leeArchivo", NULL);
        } else if (pid > 0) {
          wait(&status);
        }
      } else if (opcion == 'c') {
        pid = fork();
        if (pid == 0) {
          execl("./bin/buscaRegistro", "buscaRegistro", NULL);
        } else if (pid > 0) {
          wait(&status);
        }
      }
    } else if (opcion == 'd') {
      exit(0);
    } else {
      printf("Opcion no valida\n");
    }
  }
  return 0;
}

void mostrar_menu() {
  printf("----------- Papeleria ---------------\n");
  printf("a) Mostrar carpeta donde se encuentra el archivo a leer\n");
  printf("b) Leer el archivo del negocio\n");
  printf("c) Buscar por clave en el archivo del negocio\n");
  printf("d) Salir\n");
}
