CREATE TABLE houses (
  id          INTEGER     NOT NULL,
  kind        TEXT        NOT NULL,
  street      TEXT        NOT NULL,
  number      INTEGER     NOT NULL,
  floor       INTEGER     NOT NULL,
  postcode    TEXT        NOT NULL,
  rooms       INTEGER     NOT NULL,
  baths       INTEGER     NOT NULL,
  area        REAL        NOT NULL,

  CONSTRAINT houses_id_pk PRIMARY KEY ("id")
);
