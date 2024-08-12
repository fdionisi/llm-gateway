import { Mic, Paperclip, Send, SlashSquare } from "lucide-react";
import {
  ChangeEvent,
  FormEvent,
  forwardRef,
  KeyboardEvent,
  useCallback,
  useEffect,
  useRef,
} from "react";
import { Button } from "../atoms/button";
import { Row } from "../atoms/row";
import { Textarea } from "../atoms/textarea";

interface Prosp {
  value?: string;
  onChange: (value: string) => void;
  onSubmit: () => void;
  disabled?: boolean;
}

export const ChatInput = forwardRef<HTMLTextAreaElement, Prosp>(function ChatInput(
  { value, onChange, onSubmit, disabled = false }: Prosp,
  ref,
): JSX.Element {
  const handleSubmit = useCallback(
    (event?: FormEvent<HTMLFormElement>) => {
      event?.preventDefault();

      onSubmit();
    },
    [onSubmit],
  );

  const handleKeydown = useCallback(
    (event: KeyboardEvent<HTMLTextAreaElement>) => {
      if (event.metaKey && event.key === "Enter") {
        handleSubmit();
      }
    },
    [handleSubmit],
  );

  const handleMessageChange = useCallback(
    (event: ChangeEvent<HTMLTextAreaElement>) => onChange(event.currentTarget.value),
    [onChange],
  );

  return (
    <form onSubmit={handleSubmit} className="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8">
      <div className="relative">
        <Textarea
          ref={ref}
          value={value}
          onChange={handleMessageChange}
          onKeyDown={handleKeydown}
          disabled={disabled}
          placeholder="Ask me anything..."
        />

        <div className="absolute bottom-px inset-x-px p-2 rounded-b-lg bg-white dark:bg-neutral-900">
          <Row centered justifyBetween>
            <Row centered>
              <Button type="button">
                <SlashSquare className="w-4 h-4" />
              </Button>

              <Button type="button">
                <Paperclip className="w-4 h-4" />
              </Button>
            </Row>

            <Row centered>
              <Button type="button">
                <Mic className="w-4 h-4" />
              </Button>

              <Button type="submit" variant="primary">
                <Send className="w-4 h-4" />
              </Button>
            </Row>
          </Row>
        </div>
      </div>
    </form>
  );
});
