import {
  word_count,
  sentence_count,
  syllables_in_text,
  flesch_reading_ease_score,
} from "../pkg/kincaid";

const truncateDecimals = (number, digits) => {
  const multiplier = Math.pow(10, digits),
    adjustedNum = number * multiplier,
    truncatedNum = Math[adjustedNum < 0 ? "ceil" : "floor"](adjustedNum);

  return truncatedNum / multiplier;
};

export default function calculate() {
  const wordCount = document.getElementById("word_count");
  const sentenceCount = document.getElementById("sentence_count");
  const syllableCount = document.getElementById("syllable_count");
  const fleschReadingEase = document.getElementById("flesch_reading_ease");
  const text_input = document.getElementById("data");

  const textContent = text_input.value;

  wordCount.textContent = word_count(textContent);
  sentenceCount.textContent = sentence_count(textContent);
  syllableCount.textContent = syllables_in_text(textContent);

  const fleschReadingEaseScore = flesch_reading_ease_score(textContent);
  fleschReadingEase.textContent = !Number.isNaN(fleschReadingEaseScore)
    ? truncateDecimals(fleschReadingEaseScore, 2)
    : 0;
}

const text_input = document.getElementById("data");
text_input.addEventListener("input", calculate, false);
