# Sentiment Analysis Discord Bot

**[GitHub Project link](https://github.com/MidKnightXI/CAU_Natural_Language_and_Information_Retrieval_2023/tree/main/Final)**

## Abstract

This report describes a Discord bot designed to perform sentiment analysis on text messages and threads within a Discord server. The bot leverages the TextBlob library for sentiment analysis and the alt-profanity-check library to detect and handle inappropriate or offensive content. The goal of the bot is to provide insights into the sentiment of messages and threads, allowing users to gauge the overall tone of conversations and identify potentially negative or offensive content.

## Introduction

Initially, the intention was to integrate the Twitter API for sentiment analysis based on hashtags. However, due to recent changes in the Twitter API's pricing structure, the original plan had to be revised. Instead, this report focuses on the development of a Discord bot capable of analyzing sentiment within the context of a Discord server. The bot provides valuable information about the sentiment of individual messages and entire conversation threads, helping users understand the overall mood and tone of the discussions.

## Overall Goal

The overall goal of the Discord bot is to enable sentiment analysis within a Discord server. By analyzing the sentiment of messages and threads, the bot allows users to gain insights into the emotional tone of the conversations. It helps identify positive, negative, or neutral sentiments expressed in the text and provides an efficient way to moderate and manage inappropriate or offensive content.

## Functionality

The Discord bot includes the following key features:

1. Sentiment Analysis of Individual Messages:
    - When a message is sent within a Discord server, the bot analyzes the sentiment of the message using the TextBlob library.
    - If the message contains profane or offensive content, as determined by the alt-profanity-check library, the bot automatically deletes the message and sends a notification to the channel where the message was posted.
    - The sentiment analysis result is then provided as a response to the user who sent the message, indicating whether the sentiment is positive, negative, or neutral.

2. Sentiment Analysis of Conversation Threads:
    - Users can invoke the !analyze_thread command followed by the thread ID to analyze the sentiment of an entire conversation thread.
    - The bot retrieves all the messages within the specified thread and performs sentiment analysis on each message using TextBlob.
    - The sentiments of all messages are aggregated to calculate the average polarity of the thread.
    - Based on the average polarity, the bot responds with a message indicating whether the sentiment of the conversation thread is positive, negative, or neutral.

## Implementation
The Discord bot is implemented using the discord.py library, which provides a framework for creating Discord bots with ease. The bot utilizes the TextBlob library for sentiment analysis and the alt-profanity-check library to detect and handle offensive content.

The main components of the implementation include:

**Initialization and Setup:**

- The bot is created using the commands.Bot class from the discord.ext module, with appropriate intents enabled to access message content.
- The command prefix is set to '!' to indicate bot commands.
- Event handlers are defined for the on_ready and on_message events to handle bot initialization and message processing, respectively.

**Sentiment Analysis:**

- The sentiment analysis of individual messages is performed within the on_message event handler.
- The has_insults function checks if a message contains profane or offensive content using the alt-profanity-check library. If offensive content is detected, the message is deleted and a notification is sent to the channel.
- The sentiment analysis of individual messages is done using the TextBlob library, and the sentiment polarity is categorized as positive, negative, or neutral.

**Analyzing Conversation Threads:**

- The analyze_thread command allows users to analyze the sentiment of a conversation thread.
- The command takes a thread ID as input and fetches all the messages within the specified thread.
- Sentiment analysis is performed on each message using TextBlob, and the sentiments are aggregated to calculate the average polarity of the thread.
- Based on the average polarity, the bot responds with a message indicating the sentiment of the conversation thread.

## Conclusion

The developed Discord bot provides a valuable sentiment analysis capability within a Discord server. It allows users to gain insights into the sentiment expressed in individual messages and entire conversation threads. By automatically detecting and handling offensive content, the bot contributes to maintaining a positive and inclusive community environment. Although the original plan to integrate sentiment analysis based on Twitter hashtags had to be revised, the Discord bot serves as a reliable alternative for analyzing sentiment within the context of Discord conversations.

> Note: The code provided in this report is subject to modifications and improvements based on specific requirements and further development.

## References

- discord.py documentation: https://discordpy.readthedocs.io/
- TextBlob documentation: https://textblob.readthedocs.io/
- alt-profanity-check library: https://pypi.org/project/alt-profanity-check/