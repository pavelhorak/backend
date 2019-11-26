from sqlalchemy.ext.automap import automap_base
from sqlalchemy.orm import Session
from sqlalchemy import create_engine
import sys
import json
import os


Base = automap_base()
engine = create_engine(os.getenv("DATABASE_URL"))
Base.prepare(engine, reflect=True)
Booking = Base.classes.booking
session = Session(engine)


def get(data):
	"""
	Receive data as a dictionary containing id. 
	Return the Booking with the given id.
	"""


def post(data):
    """Create a new booking in the datatbase from the data given"""

def patch(data):
    ...


def delete(data):
    ...


methods = {"get": get, "post": post, "patch": patch, "delete": delete}
data = json.load(sys.stdin)
if len(sys.argv) < 2:
    methods["get"](data)
else:
    methods[sys.argv[1]](data)
