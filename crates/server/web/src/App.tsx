import { useEffect } from "react";
import { BrowserRouter, Routes, Route, Navigate } from "react-router-dom";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { useAuthStore } from "./store/authStore";
import Layout from "./components/Layout";
import ProtectedRoute from "./components/ProtectedRoute";
import LoginPage from "./pages/LoginPage";
import RegisterPage from "./pages/RegisterPage";
import LobbyPage from "./pages/LobbyPage";
import CreateServerPage from "./pages/CreateServerPage";
import ServerDetailPage from "./pages/ServerDetailPage";
import LinkApkPage from "./pages/LinkApkPage";
import GameplayPage from "./pages/GameplayPage";

const queryClient = new QueryClient();

function App() {
  const hydrate = useAuthStore((s) => s.hydrate);

  useEffect(() => {
    hydrate();
  }, [hydrate]);

  return (
    <QueryClientProvider client={queryClient}>
      <BrowserRouter>
        <Layout>
          <Routes>
            <Route path="/login" element={<LoginPage />} />
            <Route path="/register" element={<RegisterPage />} />
            <Route path="/" element={<ProtectedRoute><LobbyPage /></ProtectedRoute>} />
            <Route path="/servers/create" element={<ProtectedRoute><CreateServerPage /></ProtectedRoute>} />
            <Route path="/servers/:id" element={<ProtectedRoute><ServerDetailPage /></ProtectedRoute>} />
            <Route path="/servers/:id/link" element={<ProtectedRoute><LinkApkPage /></ProtectedRoute>} />
            <Route path="/link-apk" element={<ProtectedRoute><LinkApkPage /></ProtectedRoute>} />
            <Route path="/gameplay" element={<GameplayPage />} />
            <Route path="*" element={<Navigate to="/" replace />} />
          </Routes>
        </Layout>
      </BrowserRouter>
    </QueryClientProvider>
  );
}

export default App;
