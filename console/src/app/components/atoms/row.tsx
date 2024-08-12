import clsx from "clsx";
import { PropsWithChildren } from "react";

interface Props {
  centered?: boolean;
  justifyBetween?: boolean;
}
export function Row({
  children,
  justifyBetween = false,
  centered = false,
}: PropsWithChildren<Props>) {
  return (
    <div
      className={clsx(
        "flex gap-x-1",
        centered && "items-center",
        justifyBetween && "justify-between",
      )}
    >
      {children}
    </div>
  );
}
