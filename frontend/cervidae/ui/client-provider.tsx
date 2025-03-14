'use client'
import { Client, Provider, cacheExchange, fetchExchange, createClient } from 'urql';


const client = createClient({
  url: 'http://localhost:1234',
  fetchOptions: () => ({
    credentials: 'include', // ✅ Ensure cookies are sent/received
  }),
  exchanges: [cacheExchange, fetchExchange],
});

export default function ClientProvider({ children }: { children: React.ReactNode }) {
  return <Provider value={client}>{children}</Provider>;
}