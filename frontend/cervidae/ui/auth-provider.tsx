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
console.log(result)
  const login = () => { 
    redirect("/auth");
  };

  const logout = async () => {
    let test = await logoutExecuteQuery();
    console.log(test);
  };

  return (
    <AuthContext.Provider value={{ isAuthenticated: result.error == null,
     login, logout, isAdmin: result.data?.verifyToken?.isAdmin, userId: result.data?.verifyToken?.sub }}>
      {children}
    </AuthContext.Provider>
  );
}
