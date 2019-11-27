# backend
Backend

## /rgi/booking/<id>
Vrací JSON s následující strukturou
- id
- name
- description
- author
- rooms
- begin_time
- end_time
- layout
- approved

Jestliže není reservace s daným id nalezena vrací JSON s atributem error, kde chybová hláška

Rustí endpointy
FILTER
    Popis:
        Podle zadáných časů vrátí reservace které v zadaných místnostech v té době probíhají/budou probíhat
    Parametry:
        - Místnosti (rooms): místnosti jaké chceme filtrovat
            - 0 - žádná místnost
            - 1 - levá místnost
            - 2 - pravá místnost
            - 3 - oě místnosti
        - Začátek (begin_time): od kdy
        - Konec (end_time) : do kdy
    Vrací:
        - JSON soubor s parametrem výseldky ("results")
        kde je pole výsledků (rezervací v zadaných místnostech v zadaných časech)(booking dictionary)
LIST
    Popis:
        Vrátí všechny rezervace z databáze
    Parametry:
        Nebere parametry
    Vrací:
        - JSON soubor s paramterem výsledky ("results")
        kde jsou data (booking dictionary) všech rezervací