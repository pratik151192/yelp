FROM postgis/postgis:latest

ENV POSTGRES_USER=postgres
ENV POSTGRESS_PASSWORD=password
ENV POSTGRES_DB=yelp

COPY init.sql /docker-entrypoint-initdb.d/

EXPOSE 5432