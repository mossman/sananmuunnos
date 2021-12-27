CREATE VIEW likes_count as 
    SELECT first, second, count(*) from likes group by first, second ORDER BY count desc;
