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
  
 V případě chyby vrátí JSON s atributem result, který bude označovat chybu
 
 ## Ostatní endpointy
 Vrací JSON s atributem result, který označuje výsledek nebo chybu
 
 ## Tabulka výsledků
 result: 0    - všechno fungovalo
 result: 1    - nenašlo to rezervaci podle ID
 result: 2    - Už existuje rezervace ve stejném čase a ve stejné místnosti

 
 
