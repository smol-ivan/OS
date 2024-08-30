# El proposito del programa es crear cien mil registros aleatorios de una papeleria
import random

def main():
    nombre_productos = ['Cuaderno', 'Lapiz', 'Borrador', 'Plumon', 'Libro', 'Goma', 'Regla', 'Tijeras', 'Sacapuntas', 'Pegamento']

    marca_productos = ['Norma', 'Bic', 'Milan', 'Sharpie', 'Santillana', 'Maped', 'Fabber Castell', 'Staedtler', 'Faber Castell', 'Pritt']

    precio_productos = [10, 5, 3, 15, 20, 2, 13, 8, 7, 11]

    cantidad_productos = {"min": 100, "max": 800}

    # Crear un archivo de texto, si existe lo sobreescribe
    # Los identificadores de los productos son numeros enteros unicos
    try:
        with open('./data/registros.txt', 'w') as archivo:
            archivo.write('Identificador,Producto,Marca,Precio,Cantidad\n')
            for _ in range(100000):
                identificador = random.randint(1000000, 9999999)
                producto = random.choice(nombre_productos)
                marca = random.choice(marca_productos)
                precio = random.choice(precio_productos)
                cantidad = random.randint(cantidad_productos["min"], cantidad_productos["max"])

                archivo.write(f'{identificador},{producto},{marca},{precio},{cantidad}\n')
    except Exception as e:
        print("Error al crear el archivo de registros")
        print(e)
    else:
        print("Registros creados con exito")

    

if __name__ == '__main__':
    main()