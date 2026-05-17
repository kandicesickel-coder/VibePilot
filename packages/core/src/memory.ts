// packages/core/src/memory.ts
// Learning Card types and helpers

import type { LearningCard } from './agent';

export type LearningCardType = LearningCard['cardType'];

/**
 * Check if a learning card matches a query (title or trigger contains query)
 */
export function cardMatches(card: LearningCard, query: string): boolean {
  const q = query.toLowerCase();
  return (
    card.title.toLowerCase().includes(q) ||
    card.trigger.toLowerCase().includes(q) ||
    card.body.toLowerCase().includes(q)
  );
}

/**
 * Filter learning cards by type
 */
export function filterByType(cards: LearningCard[], type: LearningCardType): LearningCard[] {
  return cards.filter(c => c.cardType === type);
}

/**
 * Sort learning cards by relevance (confirmed cards first, then by date)
 */
export function sortByRelevance(cards: LearningCard[]): LearningCard[] {
  return [...cards].sort((a, b) => {
    // Confirmed cards first
    const aConfirmed = a.confirmedAt ? 1 : 0;
    const bConfirmed = b.confirmedAt ? 1 : 0;
    if (aConfirmed !== bConfirmed) return bConfirmed - aConfirmed;
    // Then by creation date (newer first)
    return b.createdAt.localeCompare(a.createdAt);
  });
}

/**
 * Generate a Learning Card title from an outcome
 */
export function titleFromOutcome(outcome: string, taskTitle: string): string {
  const prefix = outcome === 'success' ? '✅' : outcome === 'failed' ? '❌' : '⚠️';
  return `${prefix} ${taskTitle}`;
}