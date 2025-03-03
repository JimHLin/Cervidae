'use client'
import { Client, Provider, cacheExchange, fetchExchange } from 'urql';

const client = new Client({
  url: 'http://localhost:1234/',
  exchanges: [cacheExchange, fetchExchange],
});

export default function ClientProvider({ children }: { children: React.ReactNode }) {
  return <Provider value={client}>{children}</Provider>;
}