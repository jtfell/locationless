--
-- Overlaps are created as a view.
--
--

CREATE OR REPLACE VIEW matches AS

SELECT DISTINCT

-- Always want trip_a as the lower of the 2 IDs
     LEAST(trip_x.id, trip_y.id) as trip_a,
  GREATEST(trip_x.id, trip_y.id) as trip_b

FROM
    trips AS trip_x,
    trips AS trip_y,
    friendships

WHERE

    -- Trips can't be with yourself
    NOT trip_y."user" = trip_x."user"

    -- Trips in the same city
    AND trip_x.city = trip_y.city

    -- Trips overlap in timing
    AND trip_x.start_date <= trip_y.end_date
    AND trip_x.end_date   >  trip_y.start_date

    -- trip_x user is friends with trip_y user
    AND (user_a = trip_y."user" OR user_b = trip_y."user")
    AND (user_a = trip_x."user" OR user_b = trip_x."user")
    AND user_a_accepted = true
    AND user_b_accepted = true

