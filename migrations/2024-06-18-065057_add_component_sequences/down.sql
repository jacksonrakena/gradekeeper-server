ALTER TABLE course_component DROP IF EXISTS sequence_number;

DROP FUNCTION IF EXISTS generate_component_sequences();