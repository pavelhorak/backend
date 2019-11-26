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


Všechny endpointy:
  V případě chyby vrátí JSON s atributem error, ve kterém je uložena chybová hláška.
  
Ostatní endpointy vrací JSON s atributem success, který je buď True, nebo obsahuje chybovou hlášku
