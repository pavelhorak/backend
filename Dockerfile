FROM archlinux/base

# dependencies
RUN pacman -Suy --noconfirm rustup make gcc pkgconf postgresql \
python python-sqlalchemy python-psycopg2
RUN rustup install nightly
USER postgres
RUN initdb -D /var/lib/postgres/data 
USER root
RUN mkdir /run/postgresql && chown -R postgres:postgres /run/postgresql 
USER postgres
RUN pg_ctl -D /var/lib/postgres/data start
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
