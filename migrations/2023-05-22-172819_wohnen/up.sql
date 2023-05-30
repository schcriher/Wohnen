CREATE TABLE houses (
  id          INTEGER     NOT NULL,
  kind        TEXT        NOT NULL, -- kind should be a FOREIGN KEY
  street      TEXT        NOT NULL,
  number      INTEGER     NOT NULL,
  floor       INTEGER     NOT NULL,
  postcode    INTEGER     NOT NULL,
  rooms       INTEGER     NOT NULL,
  baths       INTEGER     NOT NULL,
  area        REAL        NOT NULL,

  CONSTRAINT houses_id_pk PRIMARY KEY ("id")
);

INSERT INTO houses (kind, street, number, floor, postcode, rooms, baths, area)
VALUES
("Casa",        "Calle el Ceibo",           123, 1, 4321, 3, 1, 150.5),
("Apartamento", "Avenida Bustamante",      1456, 4, 4321, 2, 1,  80.2),
("Dúplex",      "Calle San Martin",         456, 3, 4321, 3, 1, 120.6),
("Casa",        "Calle Cabo de Hornos",    2859, 2, 1758, 4, 2, 200.8),
("Loft",        "Av. Bernardo Lehman",      789, 5, 8765, 2, 1,  75.2),
("Chalet",      "Calle los Pinos",          334, 1, 5678, 6, 2, 280.0),
("Apartamento", "Calle Müller",             756, 6, 3456, 2, 1,  60.3),
("Loft",        "Avenida Chacabuco",        567, 7, 3452, 2, 1,  85.8),
("Casa",        "Calle los Lapachos",      1890, 3, 6587, 4, 2, 180.0),
("Apartamento", "Av. Corrientes",          1324, 8, 2345, 2, 1,  70.5),
("Dúplex",      "Calle Jorge Newbery",      234, 2, 3452, 4, 2, 180.3),
("Casa",        "Calle 2 Arroyos",         5679, 2, 6543, 5, 2, 220.7),
("Loft",        "Avenida Blas Parera",      434, 6, 3452, 2, 1,  95.5),
("Chalet",      "Calle Lomar del Talar",   3012, 1, 5796, 8, 3, 350.1),
("Dúplex",      "Calle el Hornero",         890, 4, 1758, 3, 2, 150.0),
("Apartamento", "Avenida Weiß",            3455, 5, 7654, 2, 1,  55.8);
