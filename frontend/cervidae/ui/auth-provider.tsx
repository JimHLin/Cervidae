"use client";
import { createContext, useState, useEffect, useContext, ReactNode } from "react";
import Cookies from "js-cookie"; // Import js-cookie
import { gql } from "urql";
import { useQuery, useMutation } from "urql";
import { redirect } from "next/navigation";

interface AuthContextType {
  isAuthenticated: boolean;
  isAdmin: boolean;
  userId: string;
  login: () => void;
  logout: () => void;
  validate: () => void;
}

const verifyString = gql`
  query verifyToken {
    verifyToken{
        sub
        isAdmin
        exp
        iat
        iss
    }
  }
`;

const logoutString = gql`
  mutation logout {
    logout
  }
`;


export const AuthContext = createContext<AuthContextType | undefined>(undefined);

export function useAuth() {
    const context = useContext(AuthContext);
    if (context === undefined) {
      throw new Error("useAuth must be used within an AuthProvider");
    }
    return context;
  }

export function AuthProvider({ children }: { children: ReactNode }) {
    const [result, reexecuteQuery] = useQuery({ query: verifyString });
    const [logoutResult, logoutExecuteQuery] = useMutation(logoutString);
  const login = () => { 
    redirect("/auth");
  };
  const logout = async () => {
    let test = await logoutExecuteQuery();
    reexecuteQuery({ requestPolicy: "network-only" });
  };

  return (
    <AuthContext.Provider value={{ isAuthenticated: !result.error && !result.fetching,
     login, logout, isAdmin: result.data?.verifyToken?.isAdmin, userId: result.data?.verifyToken?.sub, validate: reexecuteQuery }}>
      {children}
    </AuthContext.Provider>
  );
}
