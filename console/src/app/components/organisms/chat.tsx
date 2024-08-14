"use client";

import { useCallback, useEffect, useMemo, useRef, useState } from "react";
import Markdown from "react-markdown";
import { ChatInput } from "../molecules/chat-input";
import { Bot, User } from "lucide-react";
import OpenAI from "openai";
import { useUser } from "@auth0/nextjs-auth0/client";

type Role = "user" | "assistant";

type Message = {
  role: Role;
  text: string;
};

const UserMessage = ({ text }: { text: string }) => {
  return (
    <li className="py-2 sm:py-4">
      <div className="max-w-4xl px-4 sm:px-6 lg:px-8 mx-auto">
        <div className="max-w-2xl flex gap-x-2 sm:gap-x-4">
          <span className="flex-shrink-0 inline-flex items-center justify-center size-[38px] rounded-full bg-gray-600">
            <User className="text-white" />
          </span>

          <div className="grow mt-2 space-y-3">
            <p className="text-gray-800 dark:text-neutral-200">{text}</p>
          </div>
        </div>
      </div>
    </li>
  );
};

const AssistantMessage = ({ text }: { text: string }) => {
  return (
    <li className="max-w-4xl py-2 px-4 sm:px-6 lg:px-8 mx-auto flex gap-x-2 sm:gap-x-4">
      <Bot className="flex-shrink-0 w-[2.375rem] h-[2.375rem] rounded-full" />
      <div className="space-y-3">
        <Markdown>{text}</Markdown>
      </div>
    </li>
  );
};

const Message = ({ role, text }: Message) => {
  switch (role) {
    case "user": {
      return <UserMessage text={text} />;
    }
    case "assistant": {
      return <AssistantMessage text={text} />;
    }
    default: {
      return null;
    }
  }
};

export function Chat() {
  const [message, setMessage] = useState("");
  const [disabled, setDisabled] = useState(false);
  const [messages, setMessages] = useState<Message[]>([]);

  const client = useMemo(
    () =>
      new OpenAI({
        baseURL: "http://localhost:3000/api/ai/v1",
        apiKey: "fake",
        dangerouslyAllowBrowser: true,
        defaultHeaders: {
          "x-llm-provider": "anthropic_vertex_ai",
        },
      }),
    [],
  );

  const textareaRef = useRef<HTMLTextAreaElement>(null);
  useEffect(() => {
    if (textareaRef.current) {
      textareaRef.current.focus();
    }
  }, []);

  const appendMessage = useCallback((role: Role, text: string) => {
    setMessages((prevMessages) => [...prevMessages, { role, text }]);
  }, []);

  const handleSubmit = useCallback(
    async function onSubmit(): Promise<void> {
      setDisabled(true);

      const messageCopy = message;
      setMessage("");
      appendMessage("user", message);

      try {
        const response = await client.chat.completions.create({
          model: "claude-3-5-sonnet@20240620",
          messages: [
            {
              role: "user",
              content: messageCopy,
            },
          ],
        });

        appendMessage("assistant", response.choices[0].message.content as string);
      } catch (_) {
        // handle error
      } finally {
        setDisabled(false);
      }
    },
    [client, message, appendMessage],
  );

  useEffect(() => {
    if (disabled === false) {
      if (textareaRef.current) {
        textareaRef.current.focus();
      }
    }
  }, [disabled]);

  return (
    <div className="relative h-screen flex-cols">
      <div className="pb-40">
        <ul className="space-y-5">
          {messages.map((message, index) => (
            <Message key={index} role={message.role} text={message.text} />
          ))}
        </ul>
      </div>

      <footer className="fixed bottom-0 left-0 right-0 z-10 bg-white border-t border-gray-200 pt-2 pb-3 sm:pt-4 sm:pb-6 dark:bg-neutral-900 dark:border-neutral-700">
        <ChatInput
          ref={textareaRef}
          disabled={disabled}
          onChange={setMessage}
          value={message}
          onSubmit={handleSubmit}
        />
      </footer>
    </div>
  );
}
