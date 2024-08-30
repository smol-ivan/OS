import threading
import subprocess
from typing import List, Callable

def mostrar_menu()->None:
    print(f'-------Papeleria-------')
    print(f'a) Mostrar carpeta donde se ecnuentra el registro')
    print(f'b) Leer el registro de negocio')
    print(f'c) Buscar por clave en el registro')
    print(f'd) Salir')

mostrar_carpeta = lambda: subprocess.run(["ls", "-l", "data"])

lee_archivo = lambda: subprocess.run(["python3", "src/lee_archivo.py"])

busca_registro = lambda: subprocess.run(["python3", "src/busca_registro.py"])

def handle_opcion(func: Callable[[], None])->None:
    print(f'Procesando...\n')
    hilo = threading.Thread(target=func)
    try:
        hilo.start()
    except Exception as e:
        print(f'Error al ejecutar la funcion')
        print(e)
    else:
        hilo.join()
        print(f'Proceso terminado\n')

def main()->None:
    opcion:chr = ''
    opciones: List[chr] = ['a', 'b', 'c']

    while (True):
        mostrar_menu()
        print(f'Ingrese una opcion.')
        opcion = input('').lower()
        if opcion in opciones:
            if opcion == 'a':
                handle_opcion(mostrar_carpeta)
                pass
            elif opcion == 'b':
                handle_opcion(lee_archivo)
            else:
                handle_opcion(busca_registro)
        elif opcion == 'd':
            print(f'Saliendo...')
            break
        else:
            print(f'Opcion no valida')
            continue

if __name__ == '__main__':
    main()