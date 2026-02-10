# Sidebar UX Research

Research on Claude/ChatGPT sidebar design philosophy and general UX patterns.
Conducted 2026-02-10 as reference for CLOSM Probe UI improvements.

---

## Documents

| Document | Content |
|----------|---------|
| [Common Principles](./common-principles.md) | Shared design philosophy across products |
| [Claude Analysis](./claude-analysis.md) | Claude.ai sidebar structure and philosophy |
| [ChatGPT Analysis](./chatgpt-analysis.md) | ChatGPT sidebar evolution and philosophy |
| [General Patterns](./general-patterns.md) | Industry sidebar UX patterns and research |
| [CLOSM Probe Gap Analysis](./closm-gap-analysis.md) | Current state vs. best practices |

---

## Key Takeaway

All modern productivity sidebars follow the **Creation-Consumption Loop**:

```
[+ New Action]     ← Create (start something new)
─────────────
[History/Starred]  ← Resume (continue past work)
─────────────
[Settings]         ← Configure (low-frequency meta)
```

This maps to CLOSM Probe as:

| Pattern | CLOSM Probe Equivalent |
|---------|----------------------|
| New Action | Open Folder |
| History | Recent folders |
| Context | Selected celestial details |
| Settings | Theme / display options |

---

## Sources

### Claude.ai
- [Conversational AI UI Comparison 2025 - IntuitionLabs](https://intuitionlabs.ai/articles/conversational-ai-ui-comparison-2025)
- [Geist | Anthropic Brand Identity](https://geist.co/work/anthropic)
- [Anthropic Branding - Abduzeedo](https://abduzeedo.com/seamlessly-crafting-ai-branding-and-visual-identity-anthropic)
- [Claude Projects - Help Center](https://support.claude.com/en/articles/9517075-what-are-projects)
- [Claude Chat Search and Memory](https://support.claude.com/en/articles/11817273-using-claude-s-chat-search-and-memory-to-build-on-previous-context)
- [Claude Product Overview](https://claude.com/product/overview)
- [Claude Personalization Features](https://support.claude.com/en/articles/10185728-understanding-claude-s-personalization-features)
- [Claudia Interface Redesign - TestingCatalog](https://www.testingcatalog.com/claude-ai-teases-claudia-interface-with-a-sidebar-akin-to-chatgpt/)
- [Claude Cowork - TIME](https://time.com/7346545/ai-claude-cowork-code-chatbots/)
- [Claude Cowork - VentureBeat](https://venturebeat.com/orchestration/claude-cowork-turns-claude-from-a-chat-tool-into-shared-ai-infrastructure)
- [Claude Skills Frontend Design](https://claude.com/blog/improving-frontend-design-through-skills)

### ChatGPT
- [ChatGPT Sidebar Redesign Guide - AI Toolbox](https://www.ai-toolbox.co/chatgpt-management-and-productivity/chatgpt-sidebar-redesign-guide)
- [UX Principles - OpenAI Developers](https://developers.openai.com/apps-sdk/concepts/ux-principles/)
- [UI Guidelines - OpenAI Developers](https://developers.openai.com/apps-sdk/concepts/ui-guidelines)
- [Beyond the Bot - PromptEngineering.org](https://promptengineering.org/beyond-the-bot-why-chatgpts-interface-was-the-real-innovation/)
- [What Makes a Great ChatGPT App](https://developers.openai.com/blog/what-makes-a-great-chatgpt-app/)
- [Introducing Apps in ChatGPT](https://openai.com/index/introducing-apps-in-chatgpt/)
- [ChatGPT UX Case Study - shooka95k](https://shooka95k.com/portfolio-items/chat-gpt-history-and-chat-management-ux-case-study/)
- [ChatGPT Release Notes](https://help.openai.com/en/articles/6825453-chatgpt-release-notes)
- [Timeline of ChatGPT - SEJ](https://www.searchenginejournal.com/history-of-chatgpt-timeline/488370/)
- [Three Years of ChatGPT - TheNeuron](https://www.theneuron.ai/explainer-articles/three-years-of-chatgpt-a-retrospective-2022-2025/)
- [Projects in ChatGPT](https://help.openai.com/en/articles/10169521-using-projects-in-chatgpt)

### General UX Patterns
- [Left-Side Vertical Navigation - NNGroup](https://www.nngroup.com/articles/vertical-nav/)
- [Sidebar Best Practices - UX Planet](https://uxplanet.org/best-ux-practices-for-designing-a-sidebar-9174ee0ecaa2)
- [Sidebar UI Design - Mobbin](https://mobbin.com/glossary/sidebar)
- [Fitts's Law - NNGroup](https://www.nngroup.com/articles/fitts-law/)
- [Mental Models - NNGroup](https://www.nngroup.com/articles/mental-models/)
- [Progressive Disclosure - NNGroup](https://www.nngroup.com/articles/progressive-disclosure/)
- [Notion Sidebar UI Breakdown - Medium](https://medium.com/@quickmasum/ui-breakdown-of-notions-sidebar-2121364ec78d)
- [Slack New Design - Slack Help](https://slack.com/help/articles/16764236868755-An-overview-of-Slacks-new-design)
- [VS Code Activity Bar API](https://code.visualstudio.com/api/ux-guidelines/activity-bar)
- [Progressive Disclosure for AI Agents - Substack](https://aipositive.substack.com/p/progressive-disclosure-matters)
