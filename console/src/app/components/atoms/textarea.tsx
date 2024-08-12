import clsx from "clsx";
import { forwardRef, TextareaHTMLAttributes } from "react";

export const Textarea = forwardRef<
  HTMLTextAreaElement,
  TextareaHTMLAttributes<HTMLTextAreaElement>
>(function Textarea({ className, ...rest }: TextareaHTMLAttributes<HTMLTextAreaElement>, ref) {
  return (
    <textarea
      ref={ref}
      {...rest}
      className={clsx(
        "p-4 pb-12 block w-full border-gray-200 rounded-lg text-sm focus:border-blue-500 focus:ring-blue-500 disabled:opacity-50 disabled:pointer-events-none dark:bg-neutral-900 dark:border-neutral-700 dark:text-neutral-400 dark:placeholder-neutral-500 dark:focus:ring-neutral-600",
        className,
      )}
    />
  );
});
