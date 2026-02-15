import { useState, useEffect } from 'react'
import './App.css'

interface Toast {
  id: string
  message: string
  type: 'success' | 'error' | 'info'
}

interface Transaction {
  id: string
  sender: string
  receiver: string
  amount: number
  timestamp: string
  signature: string
}

interface Block {
  index: number
  timestamp: string
  transactions: Transaction[]
  previous_hash: string
  hash: string
  nonce: number
}

interface ApiResponse<T> {
  success: boolean
  data?: T
  error?: string
}

function App() {
  const [blocks, setBlocks] = useState<Block[]>([])
  const [balance, setBalance] = useState<number>(0)
  const [address, setAddress] = useState<string>('miner-address')
  const [sender, setSender] = useState<string>('user-1')
  const [receiver, setReceiver] = useState<string>('user-2')
  const [amount, setAmount] = useState<number>(10)
  const [isMining, setIsMining] = useState<boolean>(false)
  const [, setChainValid] = useState<boolean>(true)
  const [loading, setLoading] = useState<boolean>(true)
  const [toasts, setToasts] = useState<Toast[]>([])
  const [currentPage, setCurrentPage] = useState<'explorer' | 'howto'>('explorer')
  const [mobileNavOpen, setMobileNavOpen] = useState<boolean>(false)

  const API_BASE = import.meta.env.VITE_API_BASE || 'http://localhost:3000'

  const fetchBlocks = async () => {
    try {
      const response = await fetch(`${API_BASE}/blocks`)
      if (!response.ok) {
        throw new Error(`HTTP error! status: ${response.status}`)
      }
      const data: ApiResponse<Block[]> = await response.json()
      if (data.success && data.data) {
        setBlocks(data.data)
      }
    } catch (error) {
      console.error('Failed to fetch blocks:', error)
    }
  }

  const fetchBalance = async (addr: string) => {
    try {
      const response = await fetch(`${API_BASE}/balance/${addr}`)
      if (!response.ok) {
        throw new Error(`HTTP error! status: ${response.status}`)
      }
      const data: ApiResponse<number> = await response.json()
      if (data.success && data.data !== undefined) {
        setBalance(data.data)
      }
    } catch (error) {
      console.error('Failed to fetch balance:', error)
    }
  }

  const checkChainValidity = async () => {
    try {
      const response = await fetch(`${API_BASE}/chain/valid`)
      if (!response.ok) {
        throw new Error(`HTTP error! status: ${response.status}`)
      }
      const data: ApiResponse<boolean> = await response.json()
      if (data.success && data.data !== undefined) {
        setChainValid(data.data)
      }
    } catch (error) {
      console.error('Failed to check chain validity:', error)
    }
  }

  const addTransaction = async () => {
    if (!sender || !receiver || amount <= 0) {
      showToast('Please fill all fields with valid values', 'error')
      return
    }

    try {
      const response = await fetch(`${API_BASE}/transaction`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          sender,
          receiver,
          amount,
        }),
      })
      if (!response.ok) {
        throw new Error(`HTTP error! status: ${response.status}`)
      }
      const data: ApiResponse<string> = await response.json()
      if (data.success) {
        showToast('Transaction added successfully!', 'success')
        fetchBlocks()
      } else {
        showToast(data.error || 'Failed to add transaction. Please try again.', 'error')
      }
    } catch (error) {
      console.error('Failed to add transaction:', error)
      showToast('Network error: Could not add transaction. | Insufficient balance', 'error')
    }
  }

  const mineBlock = async () => {
    setIsMining(true)
    try {
      const response = await fetch(`${API_BASE}/mine`, {
        method: 'POST',
      })
      if (!response.ok) {
        throw new Error(`HTTP error! status: ${response.status}`)
      }
      const data: ApiResponse<Block> = await response.json()
      if (data.success) {
        showToast('Block mined successfully!', 'success')
        fetchBlocks()
        fetchBalance(address)
        checkChainValidity()
      } else {
        showToast(data.error || 'Failed to mine block.', 'error')
      }
    } catch (error) {
      console.error('Failed to mine block:', error)
      showToast('Network error: Mining failed. Please try again.', 'error')
    } finally {
      setIsMining(false)
    }
  }

  useEffect(() => {
    const initialize = async () => {
      setLoading(true)
      await Promise.all([
        fetchBlocks(),
        fetchBalance(address),
        checkChainValidity()
      ])
      setLoading(false)
    }
    initialize()
  }, [])

  useEffect(() => {
    fetchBalance(address)
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [address])

  const showToast = (message: string, type: 'success' | 'error' | 'info' = 'info') => {
    const id = Date.now().toString()
    const toast: Toast = { id, message, type }
    setToasts(prev => [...prev, toast])

    // Auto remove after 4 seconds
    setTimeout(() => {
      setToasts(prev => prev.filter(t => t.id !== id))
    }, 4000)
  }

  const removeToast = (id: string) => {
    setToasts(prev => prev.filter(t => t.id !== id))
  }

  // const scrollToLatestTransaction = () => {
  //   const latestBlock = blocks[blocks.length - 1]
  //   if (latestBlock) {
  //     const element = document.getElementById(`block-${latestBlock.hash}`)
  //     if (element) {
  //       element.scrollIntoView({ behavior: 'smooth', block: 'start' })
  //       // Add highlight effect
  //       element.classList.add('highlight')
  //       setTimeout(() => element.classList.remove('highlight'), 2000)
  //     }
  //   }
  // }

  if (loading) {
    return (
      <div className="app">
        <div className="loading">Loading blockchain...</div>
      </div>
    )
  }

  return (
    <div className="app">
      {/* Toast Notifications */}
      <div className="toast-container">
        {toasts.map((toast) => (
          <div key={toast.id} className={`toast toast-${toast.type}`}>
            <span className="toast-message">{toast.message}</span>
            <button onClick={() => removeToast(toast.id)} className="toast-close" aria-label="Close">×</button>
          </div>
        ))}
      </div>

      <header className="header">
        <div className="header-main">
          <button
            className={`mobile-nav-toggle ${mobileNavOpen ? 'open' : ''}`}
            onClick={() => setMobileNavOpen(!mobileNavOpen)}
            aria-label="Toggle navigation"
          >
            <span></span>
            <span></span>
            <span></span>
          </button>
          <h1>Blockchain v2</h1>
          {currentPage === 'explorer' && (
            <button
              onClick={mineBlock}
              disabled={isMining}
              className={`btn-secondary mine-btn-header ${isMining ? 'is-mining' : ''}`}
            >
              {isMining ? 'Mining Block...' : 'Mine New Block'}
            </button>
          )}
        </div>
        <nav className={`header-nav ${mobileNavOpen ? 'open' : ''}`}>
          <button
            onClick={() => {
              setCurrentPage('explorer')
              setMobileNavOpen(false)
            }}
            className={`nav-btn ${currentPage === 'explorer' ? 'active' : ''}`}
          >
            Explorer
          </button>
          <button
            onClick={() => {
              setCurrentPage('howto')
              setMobileNavOpen(false)
            }}
            className={`nav-btn ${currentPage === 'howto' ? 'active' : ''}`}
          >
            How to Use
          </button>
          <a href="https://github.com/1JOAT/blockchain" target="_blank" rel="noopener noreferrer" className="nav-btn github-nav-btn">
            GitHub
          </a>
        </nav>
      </header>

      {currentPage === 'explorer' ? (
        <div className="container">
          <div className="sidebar">
            <div className="card">
              <h2>Balance Check</h2>
              <input
                type="text"
                value={address}
                onChange={(e) => setAddress(e.target.value)}
                placeholder="Enter address"
              />
              <p className="balance">Balance: <strong>{balance} coins</strong></p>
            </div>

            <div className="card">
              <h2>Add Transaction</h2>
              <input
                type="text"
                value={sender}
                onChange={(e) => setSender(e.target.value)}
                placeholder="Sender address"
              />
              <input
                type="text"
                value={receiver}
                onChange={(e) => setReceiver(e.target.value)}
                placeholder="Receiver address"
              />
              <input
                type="number"
                value={amount}
                onChange={(e) => setAmount(Number(e.target.value))}
                placeholder="Amount"
                min="1"
              />
              <button onClick={addTransaction} className="btn-primary">
                Add Transaction
              </button>
            </div>
          </div>

          <div className="main-content">
            <div className="card">
              <h2>Blockchain ({blocks.length} blocks)</h2>
              <div className="blocks-container">
                {[...blocks].reverse().map((block) => (
                  <div key={block.hash} id={`block-${block.hash}`} className="block-card">
                    <div className="block-header">
                      <h3>Block #{block.index}</h3>
                      <small>{new Date(block.timestamp).toLocaleString()}</small>
                    </div>
                    <div className="block-details">
                      <p><strong>Hash:</strong> {block.hash.substring(0, 20)}...</p>
                      <p><strong>Prev Hash:</strong> {block.previous_hash.substring(0, 20)}...</p>
                      <p><strong>Nonce:</strong> {block.nonce}</p>
                      <p><strong>Transactions:</strong> {block.transactions.length}</p>
                    </div>
                    <div className="transactions">
                      {block.transactions.map((tx) => (
                        <div key={tx.id} className="transaction">
                          <small>
                            {tx.sender || 'MINING REWARD'} → {tx.receiver}: {tx.amount} coins
                          </small>
                        </div>
                      ))}
                    </div>
                  </div>
                ))}
              </div>
            </div>
          </div>
        </div>
      ) : (
        <div>
          <div className="main-content-full">
            <div className="card">
              <h2>How to Use This Blockchain Explorer</h2>

              <div className="howto-section">
                <h3>Getting Started</h3>
                <p>Welcome to the Blockchain Explorer! This application demonstrates a fully functional blockchain with proof-of-work mining, transaction processing, and real-time balance tracking.</p>
              </div>

              <div className="howto-section">
                <h3>Checking Balances</h3>
                <ol>
                  <li>Enter any address in the "Balance Check" field on the left sidebar</li>
                  <li>The system will display the current coin balance for that address</li>
                  <li>Try addresses like "miner-address", "user-1", "user-2" to see different balances</li>
                </ol>
              </div>

              <div className="howto-section">
                <h3>Adding Transactions</h3>
                <ol>
                  <li>Fill in the sender address (who's sending coins)</li>
                  <li>Fill in the receiver address (who's receiving coins)</li>
                  <li>Enter the amount of coins to transfer</li>
                  <li>Click "Add Transaction" to submit</li>
                  <li>Transactions are queued and will be included in the next mined block</li>
                </ol>
              </div>

              <div className="howto-section">
                <h3>Mining Blocks</h3>
                <ol>
                  <li>Click the "Mine Block" button in the header</li>
                  <li>The system will perform proof-of-work mining (this may take a few seconds)</li>
                  <li>Once mined, all pending transactions are included in the new block</li>
                  <li>The miner receives a mining reward automatically</li>
                </ol>
              </div>

              <div className="howto-section">
                <h3>Exploring the Blockchain</h3>
                <ul>
                  <li><strong>Block Details:</strong> Each block shows its index, timestamp, hash, previous hash, nonce, and transaction count</li>
                  <li><strong>Transactions:</strong> View all transactions within each block, including mining rewards</li>
                  <li><strong>Chain Status:</strong> Monitor whether the blockchain is valid (green = valid, red = invalid)</li>
                  <li><strong>Latest Block:</strong> Use the "Latest Block" button to quickly scroll to the most recent block</li>
                </ul>
              </div>

              <div className="howto-section">
                <h3>Technical Features</h3>
                <ul>
                  <li><strong>Proof-of-Work:</strong> SHA-256 mining with adjustable difficulty</li>
                  <li><strong>Transaction Validation:</strong> Ensures sufficient balance and valid addresses</li>
                  <li><strong>Chain Validation:</strong> Verifies block hashes and transaction integrity</li>
                  <li><strong>Real-time Updates:</strong> Balances and chain status update automatically</li>
                  <li><strong>Persistent Storage:</strong> Blockchain data is saved to disk</li>
                </ul>
              </div>

              <div className="howto-section">
                <h3>Try It Out!</h3>
                <p>Here's a quick demo scenario:</p>
                <ol>
                  <li>Check the balance of "user-1" (should have some coins)</li>
                  <li>Add a transaction: user-1 → user-2 for 5 coins</li>
                  <li>Mine a block to process the transaction</li>
                  <li>Check balances again to see the transfer</li>
                  <li>Explore the new block in the blockchain</li>
                </ol>
              </div>

              <div className="howto-section">
                <h3>Mobile Experience</h3>
                <p>On mobile devices, the interface automatically adjusts:</p>
                <ul>
                  <li>Balance and transaction forms appear first</li>
                  <li>Blockchain explorer follows below</li>
                  <li>Navigation is optimized for touch interaction</li>
                </ul>
              </div>
            </div>
          </div>
        </div>
      )}
    </div>
  )
}

export default App
