import Ajv from 'ajv';
import fs from 'fs';
import path from 'path';

const ajv = new Ajv();
const schema = JSON.parse(fs.readFileSync('./src/data/exercises/exercise.schema.json'));
const validate = ajv.compile(schema);

const exerciseFiles = [
  'abs.json', 'back.json', 'biceps.json', 
  'chest.json', 'glutes.json', 'hamstrings.json',
  'quads.json', 'shoulders.json', 'triceps.json'
];

exerciseFiles.forEach(file => {
  const data = JSON.parse(fs.readFileSync(`./src/data/exercises/${file}`));
  if (!validate(data)) {
    console.error(`Invalid ${file}:`, validate.errors);
    process.exit(1);
  }
});

console.log('All exercise files are valid!');
