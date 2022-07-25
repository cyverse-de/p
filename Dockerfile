FROM eclipse-temurin:18
WORKDIR /usr/src/app
COPY . /usr/src/app/

ADD https://raw.githubusercontent.com/technomancy/leiningen/stable/bin/lein /usr/bin/lein
RUN chmod a+x /usr/bin/lein
RUN lein
RUN lein do clean, install

ENTRYPOINT ["java"]
CMD ["--help"]