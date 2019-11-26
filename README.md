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
