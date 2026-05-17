// packages/core/src/context.ts
// Context Pack assembly logic

import type { ContextPack, LearningCard, Task, ContextPackComponent } from './agent';

/**
 * Assemble a Context Pack from project data.
 * Rules content goes FIRST (fixed prefix, good for prompt caching).
 * Dynamic content (learning cards, tasks) goes AFTER.
 */
export function assembleContextPack(
  projectId: string,
  rulesContent: string,
  learningCards: LearningCard[],
  activeTasks: Task[],
  repoMapSummary: string = ''
): ContextPack {
  const components: ContextPackComponent[] = [];

  // Rules (fixed prefix — benefits from Anthropic/OpenAI prompt caching)
  const rulesTokens = Math.ceil(rulesContent.length / 4);
  components.push({
    name: 'rules',
    tokenEstimate: rulesTokens,
    contentPreview: `${rulesContent.split('\n').length} lines`,
  });

  // Learning Cards
  const cardsTokens = learningCards.reduce((sum, c) => sum + Math.ceil(c.body.length / 4), 0);
  if (learningCards.length > 0) {
    components.push({
      name: 'learning_cards',
      tokenEstimate: cardsTokens,
      contentPreview: `${learningCards.length} cards`,
    });
  }

  // Active tasks
  const tasksTokens = activeTasks.reduce((sum, t) => sum + Math.ceil(JSON.stringify(t).length / 4), 0);
  if (activeTasks.length > 0) {
    components.push({
      name: 'active_tasks',
      tokenEstimate: tasksTokens,
      contentPreview: `${activeTasks.length} tasks`,
    });
  }

  const totalTokens = components.reduce((sum, c) => sum + c.tokenEstimate, 0);

  return {
    projectId,
    rulesContent,
    learningCards,
    repoMapSummary,
    activeTasks,
    totalTokensEstimate: totalTokens,
    components,
  };
}

/**
 * Estimate tokens from a string (rough: 4 chars per token for English)
 */
export function estimateTokens(text: string): number {
  return Math.ceil(text.length / 4);
}