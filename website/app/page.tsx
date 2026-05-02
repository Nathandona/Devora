import { Nav } from '@/components/layout/nav';
import { Hero } from '@/components/hero/hero';
import { FeatureGrid } from '@/components/features/feature-grid';
import { LanguageStatus } from '@/components/languages/language-status';
import { OpenSourceCTA } from '@/components/demo/open-source-cta';
import { FinalCTA } from '@/components/cta/final-cta';
import { Footer } from '@/components/layout/footer';

export default function Home() {
  return (
    <>
      <Nav />
      <main>
        <Hero />
        <FeatureGrid />
        <LanguageStatus />
        <OpenSourceCTA />
        <FinalCTA />
      </main>
      <Footer />
    </>
  );
}
