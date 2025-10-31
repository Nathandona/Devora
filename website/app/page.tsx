import { Nav } from '@/components/layout/nav';
import { Hero } from '@/components/hero/hero';
import { FeatureGrid } from '@/components/features/feature-grid';
import { LanguageGrid } from '@/components/languages/language-grid';
import { LiveTerminal } from '@/components/demo/live-terminal';
import { OpenSourceCTA } from '@/components/demo/open-source-cta';
import { FinalCTA } from '@/components/cta/final-cta';
import { Footer } from '@/components/layout/footer';

export default function Home() {
  return (
    <div className="min-h-screen bg-background">
      <Nav />
      <main>
        <Hero />
        <FeatureGrid />
        <LanguageGrid />
        <LiveTerminal />
        <OpenSourceCTA />
        <FinalCTA />
      </main>
      <Footer />
    </div>
  );
}
