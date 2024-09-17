import tkinter as tk # General logic
from tkinter import ttk # Widgets
import ttkbootstrap as ttk # Tema

window = ttk.Window()
window.title("Papeleria")
window.geometry("300x300")

text_label = ttk.Label(master=window,
    text="La libreria X cuenta con los siguientes productos:",
    wraplength=250,
    justify="center",
    font=("Menlo", 12)
)
text_label.pack(pady=15)

# field de libros
info_libros = ttk.Frame(master=window)
#TODO: Crear un laber,tabla,treeview o listbox con los libros

# field input
input_frame = ttk.Frame(master=window)
entry_int = tk.IntVar() # Variable para guardar el valor del entry
entry = ttk.Entry(master=input_frame, textvariable=entry_int)
btn = ttk.Button(master=input_frame, text="Ok")
entry.pack(side = tk.LEFT, padx=10)
btn.pack(side = tk.LEFT, padx=10)
input_frame.pack(pady=30)

window.mainloop()
