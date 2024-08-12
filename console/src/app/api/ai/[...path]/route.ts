import { getAccessToken, withApiAuthRequired } from "@auth0/nextjs-auth0";
import { NextResponse } from "next/server";

export const POST = withApiAuthRequired(async function POST(req) {
  const res = new NextResponse();
  const { accessToken } = await getAccessToken(req, res);
  const { url, method } = req;
  const u = new URL(url);
  const su = `http://localhost:3001/${u.pathname.replaceAll("/api/ai/", "")}`;
  const response = await fetch(su, {
    method,
    duplex: "half",
    headers: {
      "content-type": "application/json",
      authorization: `Bearer ${accessToken}`,
    },
    body: req.body,
  } as any);

  const reader = await response.json();
  return Response.json(reader);
});
