FROM archlinux/base

# dependencies
RUN pacman -Suy --noconfirm rustup make gcc pkgconf postgresql \
python python-sqlalchemy python-psycopg2
RUN rustup install nightly
USER postgres
RUN initdb -D /var/lib/postgres/data && pg_ctl -D /var/lib/postgres/data -l logfile start
USER root

# workdir
WORKDIR /cw

# copy to workdir
COPY . .

# build
RUN make

# expose ports
EXPOSE 8000

# run cmd
CMD ["cargo", "run", "--release"]
