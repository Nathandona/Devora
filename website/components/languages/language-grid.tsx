'use client';

import { motion } from 'framer-motion';
import { Badge } from '@/components/ui/badge';
import { Separator } from '@/components/ui/separator';
import { LanguageCard } from './language-card';
import { fadeInUp, staggerChildren } from '@/lib/animations';
import { LANGUAGES } from '@/lib/constants';

export function LanguageGrid() {
  const availableLanguages = LANGUAGES.filter(lang => lang.status === 'available');
  const comingSoonLanguages = LANGUAGES.filter(lang => lang.status === 'coming-soon');
  const plannedLanguages = LANGUAGES.filter(lang => lang.status === 'planned');

  return (
    <section id="languages" className="py-20 px-4 bg-muted/20">
      <div className="container max-w-7xl mx-auto">
        <motion.div
          initial="initial"
          whileInView="animate"
          viewport={{ once: true }}
          variants={staggerChildren}
          className="text-center space-y-4 mb-16"
        >

          <motion.h2
            variants={fadeInUp}
            className="text-3xl md:text-5xl font-bold tracking-tight"
          >
            Works with Any Language
          </motion.h2>

          <motion.p
            variants={fadeInUp}
            className="text-lg md:text-xl text-muted-foreground max-w-3xl mx-auto"
          >
            From systems programming to web development.
            Extensible architecture means adding new languages is simple.
          </motion.p>
        </motion.div>

        {/* Available Languages */}
        {availableLanguages.length > 0 && (
          <div className="mb-16">
            <motion.div
              initial={{ opacity: 0, x: -20 }}
              whileInView={{ opacity: 1, x: 0 }}
              viewport={{ once: true }}
              className="flex items-center gap-3 mb-8"
            >
              <div className="w-2 h-2 rounded-full bg-green-500" />
              <h3 className="text-xl font-semibold">Available Now</h3>
              <Badge variant="secondary">{availableLanguages.length} language{availableLanguages.length !== 1 ? 's' : ''}</Badge>
            </motion.div>

            <motion.div
              initial="initial"
              whileInView="animate"
              viewport={{ once: true }}
              variants={staggerChildren}
              className="grid md:grid-cols-2 lg:grid-cols-3 gap-6"
            >
              {availableLanguages.map((language, index) => (
                <LanguageCard key={language.name} language={language} index={index} />
              ))}
            </motion.div>
          </div>
        )}

        {/* Coming Soon */}
        {comingSoonLanguages.length > 0 && (
          <div className="mb-16">
            <motion.div
              initial={{ opacity: 0, x: -20 }}
              whileInView={{ opacity: 1, x: 0 }}
              viewport={{ once: true }}
              transition={{ delay: 0.2 }}
              className="flex items-center gap-3 mb-8"
            >
              <div className="w-2 h-2 rounded-full bg-yellow-500" />
              <h3 className="text-xl font-semibold">Coming Soon</h3>
              <Badge variant="secondary">{comingSoonLanguages.length} language{comingSoonLanguages.length !== 1 ? 's' : ''}</Badge>
            </motion.div>

            <motion.div
              initial="initial"
              whileInView="animate"
              viewport={{ once: true }}
              variants={staggerChildren}
              transition={{ delay: 0.2 }}
              className="grid md:grid-cols-2 lg:grid-cols-3 gap-6"
            >
              {comingSoonLanguages.map((language, index) => (
                <LanguageCard key={language.name} language={language} index={index} />
              ))}
            </motion.div>
          </div>
        )}

        {/* Planned Languages */}
        {plannedLanguages.length > 0 && (
          <div>
            <motion.div
              initial={{ opacity: 0, x: -20 }}
              whileInView={{ opacity: 1, x: 0 }}
              viewport={{ once: true }}
              transition={{ delay: 0.4 }}
              className="flex items-center gap-3 mb-8"
            >
              <div className="w-2 h-2 rounded-full bg-muted-foreground" />
              <h3 className="text-xl font-semibold">Planned for Future</h3>
              <Badge variant="outline">{plannedLanguages.length} language{plannedLanguages.length !== 1 ? 's' : ''}</Badge>
            </motion.div>

            <motion.div
              initial="initial"
              whileInView="animate"
              viewport={{ once: true }}
              variants={staggerChildren}
              transition={{ delay: 0.4 }}
              className="grid md:grid-cols-2 lg:grid-cols-3 gap-6"
            >
              {plannedLanguages.map((language, index) => (
                <LanguageCard key={language.name} language={language} index={index} />
              ))}
            </motion.div>
          </div>
        )}

        {/* Contribute CTA */}
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          whileInView={{ opacity: 1, y: 0 }}
          viewport={{ once: true }}
          transition={{ delay: 0.6 }}
          className="mt-20 text-center"
        >
          <div className="p-8 rounded-xl border border-border/50 bg-card/30 backdrop-blur-sm">
            <h3 className="text-2xl font-semibold mb-4">Want to add a language?</h3>
            <p className="text-muted-foreground mb-6 max-w-2xl mx-auto">
              Devora's plugin architecture makes it easy to add support for new languages.
              Check out our contribution guide to learn how to add your favorite language.
            </p>
            <div className="flex justify-center">
              <button
                onClick={() => window.open('https://github.com/Nathandona/devora/blob/main/CONTRIBUTING.md', '_blank')}
                className="px-6 py-3 bg-primary hover:bg-primary/90 text-primary-foreground rounded-lg font-medium transition-colors"
              >
                Contribute a Language
              </button>
            </div>
          </div>
        </motion.div>
      </div>
    </section>
  );
}