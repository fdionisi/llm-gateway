import clsx from "clsx";
import { ButtonHTMLAttributes, PropsWithChildren } from "react";

interface Props extends PropsWithChildren<ButtonHTMLAttributes<HTMLButtonElement>> {
  variant?: "primary" | "secondary";
}

export function Button({
  children,
  variant = "secondary",
  className,
  ...rest
}: Props): JSX.Element {
  return (
    <button
      {...rest}
      className={clsx(
        variant === "secondary" &&
          "text-gray-500 hover:text-blue-600 dark:text-neutral-500 dark:hover:text-blue-500",
        variant === "primary" && "text-white bg-blue-600 hover:bg-blue-500",
        "inline-flex flex-shrink-0 justify-center items-center size-8 rounded-lg focus:z-10 focus:outline-none focus:ring-2 focus:ring-blue-500",
        className,
      )}
    >
      {children}
    </button>
  );
}
