from typing import List, Optional

def binary_search(registros: List[List[str]], busqueda: str) -> Optional[List[str]]:
    low, high = 0, len(registros) - 1
    while low <= high:
        mid = (low + high) // 2
        if registros[mid][0] == busqueda:
            return registros[mid]
        elif registros[mid][0] < busqueda:
            low = mid + 1
        else:
            high = mid - 1
    return None

def main()->None:
    print("Ingrese el código a buscar: ")
    busqueda:str = input("")

    try:
        with open('data/registros.txt', 'r') as archivo:
            registros:List[str] = archivo.readlines()
    except Exception as e:
        print("Error al leer el archivo de registros")
        print(e)
    else:
        # Separar cabecera de registros y convertir registros en listas
        header = registros[0].split(',')
        registros = registros[1:]
        registros:List[List[str]] = [registro.split(',') for registro in registros]
        
        # Ordenar registros por ID
        registros.sort(key=lambda x: x[0])

        # Busqueda binaria
        registro = binary_search(registros, busqueda)

        if registro:
            print(f'{header[0]:<15}{header[1]:<15}{header[2]:<15}{header[3]:<15}{header[4]:<15}\n')
            print(f'{registro[0]:<15}{registro[1]:<15}{registro[2]:<15}{registro[3]:<15}{registro[4]:<15}\n')
        else:
            print("Registro no encontrado")


if __name__ == '__main__':
    main()




