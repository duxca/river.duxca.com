import { test, expect } from '@playwright/test';

test.describe('Map Interactions', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await page.waitForLoadState('networkidle');
    
    // Wait for map to be ready
    await expect(page.locator('.leaflet-container')).toBeVisible();
  });

  test('should be able to zoom in and out', async ({ page }) => {
    const zoomInButton = page.locator('.leaflet-control-zoom-in');
    const zoomOutButton = page.locator('.leaflet-control-zoom-out');
    
    await expect(zoomInButton).toBeVisible();
    await expect(zoomOutButton).toBeVisible();
    
    // Test zoom in
    await zoomInButton.click();
    await page.waitForTimeout(500); // Wait for zoom animation
    
    // Test zoom out
    await zoomOutButton.click();
    await page.waitForTimeout(500); // Wait for zoom animation
  });

  test('should display map attribution', async ({ page }) => {
    // Check that map attribution is visible
    const attribution = page.locator('.leaflet-control-attribution');
    await expect(attribution).toBeVisible();
  });
});