CREATE EXTENSION IF NOT EXISTS postgis;
CREATE EXTENSION IF NOT EXISTS pg_trgm;

CREATE TABLE IF NOT EXISTS Users(
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    email VARCHAR(100) NOT NULL,
    phone VARCHAR(15)
);

CREATE TABLE IF NOT EXISTS Businesses(
    id SERIAL PRIMARY KEY,
    name VARCHAR(150) NOT NULL,
    description TEXT NOT NULL,
    category VARCHAR(50),
    location GEOMETRY(Point, 4326),
    address TEXT,
    avg_rating REAL CHECK (avg_rating >= 0 AND avg_rating <= 5),
    num_rating INT DEFAULT 0
);

-- Create a GiST index for geospatial queries on location
CREATE INDEX IF NOT EXISTS idx_location ON Businesses USING GIST(location);

-- Create a GIN index for full-text search on the "name" column
CREATE INDEX IF NOT EXISTS idx_name_fulltext ON Businesses USING GIN (to_tsvector('english', name));

-- Create a B-Tree index for the "category" column
CREATE INDEX IF NOT EXISTS idx_category ON Businesses USING BTREE(category);

CREATE TABLE IF NOT EXISTS Reviews(
    id SERIAL PRIMARY KEY,
    user_id INT NOT NULL,
    business_id INT NOT NULL,
    rating REAL CHECK (rating >= 0 AND rating <= 5),
    comment TEXT,
    UNIQUE (user_id, business_id),
    FOREIGN KEY (user_id) REFERENCES Users(id) ON DELETE CASCADE,
    FOREIGN KEY (business_id) REFERENCES Businesses(id) ON DELETE CASCADE
);


-- Below is heavily LLM generated as my goal for this repository was not to master SQL in any way.
-- It is simply to seed data into the tables so that I can start playing around search queries.

INSERT INTO Users (name, email, phone)
SELECT
    'User ' || i,
    'user' || i || '@example.com',
    '123-456-789' || (i % 10)
FROM generate_series(1, 50) AS s(i);


INSERT INTO Businesses (name, description, category, location, address)
SELECT
    'Business ' || i,
    'Description for Business ' || i,
    CASE WHEN i % 3 = 0 THEN 'Food'
         WHEN i % 3 = 1 THEN 'Tech'
         ELSE 'Retail' END,
    ST_SetSRID(ST_MakePoint(-75.16 + random() * 10, 39.95 + random() * 10), 4326),
    'Address ' || i
FROM generate_series(1, 20) AS s(i);

-- Insert mock reviews and calculate avg_rating
DO $$
DECLARE
    business_id INT;
    num_reviews INT;
    rating_sum REAL;
    review_count INT;
    rating REAL;
    user_id INT;
    used_users INT[] := '{}'; -- Keep track of users who have already reviewed a business
BEGIN
    FOR business_id IN SELECT id FROM Businesses LOOP
        num_reviews := floor(random() * 50 + 1); -- Random number of reviews (1-50)
        review_count := 0;
        rating_sum := 0;

        used_users := '{}'; -- Reset used users for each business

        FOR review_count IN 1..num_reviews LOOP
            -- Generate a unique user_id
            LOOP
                user_id := floor(random() * 50 + 1);
                EXIT WHEN NOT (user_id = ANY (used_users)); -- Ensure no duplicates
            END LOOP;

            -- Add user_id to the used_users array
            used_users := array_append(used_users, user_id);

            -- Generate a random rating
            rating := round(random() * 5 * 10) / 10;

            rating_sum := rating_sum + rating;

            INSERT INTO Reviews (user_id, business_id, rating, comment)
            VALUES (
                user_id,
                business_id,
                rating,
                'Review for Business ' || business_id || ' by User ' || user_id
            );
        END LOOP;

        -- Update avg_rating and num_rating for the business
        UPDATE Businesses
        SET avg_rating = rating_sum / num_reviews,
            num_rating = num_reviews
        WHERE id = business_id;
    END LOOP;
END $$;
