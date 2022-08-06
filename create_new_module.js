import { writeFileSync} from 'fs';
import {
  snakeCase,
} from "change-case";
import prompts from 'prompts';

const questions = [
  {
    type: 'text',
    name: 'challenge',
    message: 'What is the challenge name',
  },
  {
    type: 'select',
    name: 'level',
    message: `What is the challenge level?`,
    choices: [
      {title: 'Easy', value: 'easy',},
      {title: 'Medium', value: 'medium',},
      {title: 'Hard', value: 'hard',},
      {title: 'Expert', value: 'expert',},
    ]
  }
]

  const main = async () => {
    const response = await prompts(questions);
    writeFileSync(`./src/${response.level}/${response.challenge}.js`, `const ${response.challenge} = () => {}\n export default ${response.challenge}`)
    writeFileSync(`./tests/${response.challenge}.test.js`, `import { ${response.challenge} } from "../src/index.js";\nimport { expect, it } from "bun:test";`)
    writeFileSync(`./src/${response.level}/${snakeCase(response.challenge)}.rs`, `pub fn ${snakeCase(response.challenge)}() -> {}\n#[cfg(test)]\nmod tests {\nuse super::${snakeCase(response.challenge)};\n#[test]\nfn test_${snakeCase(response.challenge)}() {\n}\n}`)
  }
  main()