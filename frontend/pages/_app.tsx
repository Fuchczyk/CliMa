import '@/styles/globals.css'
import type { AppProps } from 'next/app'
import { Navbar } from '@/components';

export default function App({ Component, pageProps }: AppProps) {
  return (
    <>
      <Navbar />
      <div className="min-h-screen bg-background flex flex-row">
        <Component {...pageProps} />
      </div>
    </>
  );
}
