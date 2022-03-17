
INSERT INTO users VALUES
(
  764212611,
  'Julian',
  'Fell',
  'jtfell', 
  'https://t.me/i/userpic/320/jtfell.jpg'
),
(
  764212610,
  'John',
  'Doe',
  'highflyer', 
  'https://t.me/i/userpic/320/jtfell.jpg'
),
(
  764212211,
  'Kate',
  'Smith',
  'smithy22', 
  'https://t.me/i/userpic/320/jtfell.jpg'
),
(
  730516407,
  'Peter',
  'Fell',
  'peterfell',
  'https://t.me/i/userpic/320/peterfell.jpg'
);

INSERT INTO trips VALUES 
  (100, 764212611, 2147714, DATE '2018-10-10', DATE '2018-11-10'),
  (101, 764212611, 2172517, DATE '2018-11-10', DATE '2018-12-10'),
  (102, 764212611,  587084, DATE '2018-12-10', DATE '2018-12-31'),
  (300, 730516407, 2147714, DATE '2018-10-25', DATE '2019-11-05'),
  (301, 730516407, 587084,  DATE '2018-12-05', DATE '2019-12-05');

INSERT INTO friendships VALUES
   (730516407, 764212611, true, false);

