import readline from 'readline';
import bs58 from 'bs58';

// Function to read base58 input and print decoded wallet array
function base58ToWallet(): void {
  const rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout,
  });

  rl.question('Enter your base58 private key:\n', (base58: string) => {
    try {
      const decoded: Uint8Array = bs58.decode(base58.trim());
      const wallet: number[] = Array.from(decoded);
      console.log(wallet);
    } catch (err:any) {
      console.error('❌ Invalid base58 string:', err.message);
    } finally {
      rl.close();
    }
  });
}

base58ToWallet();
