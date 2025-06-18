import { test, expect } from '@playwright/test';

test.describe('Basic Navigation', () => {
  test('should load the homepage', async ({ page }) => {
    await page.goto('/');
    
    // Wait for the page to load
    await page.waitForLoadState('networkidle');
    
    // Check that the page loaded successfully
    await expect(page).toHaveTitle(/river/i);
  });

  test('should automatically login in test mode', async ({ page }) => {
    await page.goto('/');
    
    // Wait for auto-login to complete
    await page.waitForLoadState('networkidle');
    
    // Check that we are logged in by looking for elements that only appear when logged in
    // This might be the map, navigation elements, or user info
    await expect(page.locator('body')).toContainText('Test User');
  });

  test('should display the map', async ({ page }) => {
    await page.goto('/');
    await page.waitForLoadState('networkidle');
    
    // Wait for the map container to be present
    await expect(page.locator('.leaflet-container')).toBeVisible();
  });

  test('should be able to navigate between sections', async ({ page }) => {
    await page.goto('/');
    await page.waitForLoadState('networkidle');
    
    // Look for navigation elements (buttons, links, etc.)
    // This test will need to be updated based on the actual UI structure
    const addRiverButton = page.locator('text=川を追加');
    if (await addRiverButton.isVisible()) {
      await addRiverButton.click();
      await expect(page.locator('text=川の名前')).toBeVisible();
    }
  });
});