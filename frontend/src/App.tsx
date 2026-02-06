import { useState } from 'react'
import { Routes, Route } from 'react-router-dom'
import { TopBar } from './components/TopBar'
import { AuthModal } from './components/AuthModal'
import { HomePage } from './pages/HomePage'
import { GamesPage } from './pages/GamesPage'
import { GamePage } from './pages/GamePage'

function App() {
  const [showAuthModal, setShowAuthModal] = useState(false)

  return (
    <div className="min-h-screen">
      <TopBar onLoginClick={() => setShowAuthModal(true)} />
      <AuthModal isOpen={showAuthModal} onClose={() => setShowAuthModal(false)} />

      <Routes>
        <Route path="/" element={<HomePage />} />
        <Route path="/games" element={<GamesPage />} />
        <Route path="/games/:id" element={<GamePage />} />
      </Routes>
    </div>
  )
}

export default App
