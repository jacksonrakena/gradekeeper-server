ALTER TABLE course_component ADD sequence_number SMALLINT CHECK (sequence_number > 0 and sequence_number < 1000) NULL DEFAULT NULL;

create or replace function generate_component_sequences() returns void as
$$
DECLARE t_course course%rowtype;
DECLARE t_component course_component%rowtype;
DECLARE seq int;
BEGIN
    FOR t_course in SELECT * FROM course LOOP
        seq = 1;
        FOR t_component IN SELECT * FROM course_component where course_id = t_course.id LOOP
            UPDATE course_component SET sequence_number = seq WHERE id = t_component.id;
            seq = seq + 1;
        END LOOP;
    END LOOP;
end;
$$
language plpgsql;

select generate_component_sequences() as output;